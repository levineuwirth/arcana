//! Arcade Gannon — `{2}{W}{U}` 2/3 Legendary Human Doctor.
//!
//! * `{T}: Draw a card, then discard a card. Put a quest counter on Arcade Gannon.`
//! * For Auld Lang Syne — cast an artifact/Human spell from your graveyard with mv
//!   <= quest counters. GAP'd: cast-from-graveyard permission keyed on counter count
//!   is not expressible.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcade Gannon");
    let human = reg.interner_mut().intern("Human");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(doctor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "For Auld Lang Syne" graveyard-cast permission is not expressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then discard a card. Put a quest counter on Arcade Gannon.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot_and_quest,
            }),
    )
}

fn loot_and_quest(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::AddCounters { target: ctx.source, kind: CounterKind::Quest, count: 1 },
    ]
}
