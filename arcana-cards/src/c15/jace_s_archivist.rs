//! Jace's Archivist — `{1}{U}{U}` 2/2 Vedalken Wizard.
//! `{U}, {T}: Each player discards their hand, then draws cards equal to the greatest number
//! of cards a player discarded this way.`
//! GAP: "draw cards equal to the greatest number discarded" — dynamic amount based on
//! comparing hand sizes pre-discard is not accessible after Effect::Discard resolves.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace's Archivist");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Each player discards their hand, then draws cards equal to the greatest number of cards a player discarded this way.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: wheel,
            }),
    )
}

fn wheel(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Pre-compute hand sizes to determine draw count (greatest hand size)
    let players = script::all_players(state);
    let max_hand = players.iter()
        .map(|&p| script::hand_size(state, p))
        .max()
        .unwrap_or(0);
    let mut effects: Vec<Effect> = players.iter()
        .map(|&p| Effect::Discard {
            player: p,
            count: script::hand_size(state, p),
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    // Draw max_hand cards for each player
    for p in script::all_players(state) {
        effects.push(Effect::DrawCards { player: p, count: max_hand });
    }
    effects
}
