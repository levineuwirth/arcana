//! Erebos's Emissary — `{3}{B}` 3/3 Enchantment Creature — Snake.
//! "Bestow {5}{B}
//!  Discard a creature card: This creature gets +2/+2 until end of turn.
//!  If this permanent is an Aura, enchanted creature gets +2/+2 until end
//!  of turn instead.
//!  Enchanted creature gets +3/+3."
//!
//! Bestow is not an expressible KeywordAbility (`keywords: vec![]`).
//! The discard-a-creature-card activation pumps this creature +2/+2; the
//! "if this is an Aura, enchanted creature instead" rider and the static
//! "Enchanted creature gets +3/+3" are bestow/aura statics that have no
//! demonstrated hook on this shape and are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Erebos's Emissary");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        // GAP: Bestow {5}{B} is not an expressible KeywordAbility.
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP (static): "Enchanted creature gets +3/+3" — no demonstrated
    // aura-static hook on this shape.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Discard a creature card: This creature gets +2/+2 until end of turn."
                .into(),
            cost: ActivationCost {
                discard_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_self,
        }),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if this permanent is an Aura, enchanted creature gets +2/+2
    // instead" — no aura-state branch hook; always pumps this creature.
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
