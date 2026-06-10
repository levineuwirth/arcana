//! Cavernous Maw — nonbasic land — Cave (The Lost Caverns of Ixalan,
//! 2023). "{T}: Add {C}." and "{2}: This land becomes a 3/3 Elemental
//! creature until end of turn. It's still a Cave land. Activate only if
//! the number of other Caves you control plus the number of Cave cards in
//! your graveyard is three or greater."
//! GAP: the "Activate only if …" precondition is not expressible (no
//! activation-condition hook in this API). GAP: gaining the Elemental
//! creature subtype is not expressible. The animation keeps existing
//! types (AddType is additive), so "still a Cave land" is preserved.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cavernous Maw");
    let cave = reg.interner_mut().intern("Cave");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cave);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This land becomes a 3/3 Elemental creature \
                       until end of turn. It's still a Cave land. Activate \
                       only if the number of other Caves you control plus \
                       the number of Cave cards in your graveyard is three \
                       or greater."
                    .into(),
                // GAP: the "Activate only if …" Cave-count precondition is
                // not expressible with ActivationCost.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: gaining the Elemental creature subtype is not expressible.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        },
    ]
}
