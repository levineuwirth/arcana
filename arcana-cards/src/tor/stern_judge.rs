//! Stern Judge — `{2}{W}` 2/2 white Human Cleric.
//! "{T}: Each player loses 1 life for each Swamp they control."
//!
//! GAP: Counting lands by subtype per player and applying proportional
//! life loss — script::count_matching counts battlefield permanents but
//! checking "Swamp" subtype per player requires iterating all players.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stern Judge");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let swamp_filter = arcana_core::targets::ObjectFilter::new().with_subtypes_any(vec![swamp]);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Each player loses 1 life for each Swamp they control.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_player_swamp_loss,
            }),
    )
}

fn each_player_swamp_loss(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").expect("Swamp interned");
    let swamp_filter = arcana_core::targets::ObjectFilter::new().with_subtypes_any(vec![swamp]);
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .filter_map(|p| {
            let n = script::count_matching(state, &swamp_filter, p);
            if n > 0 {
                Some(Effect::LoseLife { player: p, amount: n })
            } else {
                None
            }
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
