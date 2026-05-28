//! Kwain, Itinerant Meddler — `{W}{U}` 1/3 Legendary Rabbit Wizard.
//! `{T}:` Each player may draw a card, then each player who drew a card this
//! way gains 1 life.
//! GAP: "each player MAY draw" — optional per-player draw not expressible;
//! emitting unconditional draw + gain life for all players.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kwain, Itinerant Meddler");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Each player may draw a card, then each player who drew a card this way gains 1 life.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: group_draw,
            }),
    )
}

fn group_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player MAY draw" — optional per-player draw not expressible;
    // all players draw and gain life unconditionally.
    let all_players = script::all_players(state);
    let mut effects: Vec<Effect> = all_players.iter()
        .map(|&p| Effect::DrawCards { player: p, count: 1 })
        .collect();
    for p in &all_players {
        effects.push(Effect::GainLife { player: *p, amount: 1 });
    }
    effects
}
