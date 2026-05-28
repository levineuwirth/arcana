//! Selvala, Explorer Returned — `{1}{G}{W}` 2/4 green/white Legendary Elf Scout.
//! "Parley — {T}: Each player reveals the top card. For each nonland revealed, add {G}
//! and gain 1 life. Then each player draws a card."
//! GAP: Parley (reveal each player's top card, count nonlands) not in Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Selvala, Explorer Returned");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Parley — {T}: Each player reveals the top card. For each nonland, add {G} and gain 1 life. Each player draws a card.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: parley,
            }),
    )
}

fn parley(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Parley not fully modelable; each player draws as approximation.
    let all = script::all_players(state);
    let mut effects: Vec<Effect> = all
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect();
    // Add mana for controller (approximation of nonland cards)
    effects.push(Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    });
    effects.push(Effect::GainLife { player: ctx.controller, amount: 1 });
    effects
}
