//! Biophagus — `{1}{G}` 1/3 green Human Tyranid Wizard.
//! "Genomic Enhancement — {T}: Add one mana of any color. If this mana is spent to cast a creature
//! spell, that creature enters with an additional +1/+1 counter on it."
//!
//! GAP: The "if this mana is spent to cast a creature spell, that creature enters with +1/+1 counter"
//! rider is a replacement effect on mana spending — not expressible. Wiring the tap-for-any-color
//! as a mana ability with a generic colorless approximation (no "any color" selection in ManaUnit).
//! GAP: ManaUnit::plain requires a specific ManaColor; "any color" mana is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biophagus");
    let human = reg.interner_mut().intern("Human");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(tyranid);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_color_mana,
            }),
    )
}

fn add_any_color_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any color" mana not expressible — using colorless as placeholder.
    // The +1/+1 counter rider on creature cast is also not expressible (replacement effect).
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
