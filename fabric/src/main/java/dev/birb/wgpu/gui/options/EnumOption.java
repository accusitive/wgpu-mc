package dev.birb.wgpu.gui.options;

import dev.birb.wgpu.gui.widgets.EnumWidget;
import dev.birb.wgpu.gui.widgets.Widget;
import net.minecraft.text.LiteralText;
import net.minecraft.text.Text;

import java.lang.reflect.InvocationTargetException;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Supplier;

public class EnumOption<T extends Enum<?>> extends Option<T> {
    public final Function<T, Text> formatter;

    private final T[] values;

    @SuppressWarnings("unchecked")
    private EnumOption(Text name, Text tooltip, boolean requiresRestart, Supplier<T> getter, Consumer<T> setter, Function<T, Text> formatter) {
        super(name, tooltip, requiresRestart, getter, setter);

        this.formatter = formatter;

        try {
            values = (T[]) getter.get().getClass().getMethod("values").invoke(null);
        } catch (IllegalAccessException | InvocationTargetException | NoSuchMethodException e) {
            throw new RuntimeException(e);
        }
    }

    public T cycle(int direction) {
        for (int i = 0; i < values.length; i++) {
            if (values[i] == get()) {
                i += direction;

                if (i >= values.length) i = 0;
                else if (i < 0) i = values.length - 1;

                return values[i];
            }
        }

        throw new RuntimeException("This should never happen");
    }

    @Override
    public Widget createWidget(int x, int y, int width) {
        return new EnumWidget<>(x, y, width, this);
    }

    public static class Builder<T extends Enum<?>> extends Option.Builder<Builder<T>, T> {
        private Function<T, Text> formatter = t -> new LiteralText(t.toString());

        public Builder<T> setFormatter(Function<T, Text> formatter) {
            this.formatter = formatter;
            return this;
        }

        @Override
        public Option<T> build() {
            return new EnumOption<>(name, tooltip, requiresRestart, getter, setter, formatter);
        }
    }
}
