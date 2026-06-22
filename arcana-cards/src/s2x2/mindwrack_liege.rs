//! Mindwrack Liege — `{3}{U/R}{U/R}{U/R}` 4/4 Horror (red/blue).
//! "Other blue creatures you control get +1/+1."
//! "Other red creatures you control get +1/+1."
//! "{U/R}{U/R}{U/R}{U/R}: You may put a blue or red creature card from
//!  your hand onto the battlefield."
//!
//! The two anthem lines are static continuous buffs with no demonstrated
//! triggered/activated form — GAP'd. The activation puts a creature card
//! from hand onto the battlefield via `Effect::PutFromHandOntoBattlefield`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: static "Other blue creatures you control get +1/+1" — a continuous
// anthem, not a triggered/activated ability.
// GAP: static "Other red creatures you control get +1/+1" — likewise.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mindwrack Liege");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U/R}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U/R}{U/R}{U/R}{U/R}: You may put a blue or red creature card \
                   from your hand onto the battlefield."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U/R}{U/R}{U/R}{U/R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_creature_from_hand,
        }),
    )
}

fn put_creature_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (filter): "blue OR red" is a color disjunction the ObjectFilter
    // builders express only as AND; the creature-card put itself is wired,
    // restricted to creature cards (the player still chooses which one).
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
