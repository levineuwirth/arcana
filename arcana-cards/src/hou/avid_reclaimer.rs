//! Avid Reclaimer — `{2}{G}` 2/2 green Human Druid.
//! "{T}: Add {G} or {U}. If you control a Nissa planeswalker, you gain 2 life."
//! GAP: "Add {G} or {U}" — player choice of mana color not in Effect::AddMana.
//! GAP: "if you control a Nissa planeswalker" — no filter for specific named planeswalker.
//! Using green mana as placeholder.

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
    let name = reg.interner_mut().intern("Avid Reclaimer");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G} or {U}. If you control a Nissa planeswalker, you gain 2 life.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                // GAP: "Add {G} or {U}" has player choice; also side effect conditional on
                // controlling Nissa. This is not purely a mana ability (has a rider).
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_mana_maybe_life,
            }),
    )
}

fn add_mana_maybe_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add {G} or {U}" — using {G} as placeholder (no choice mechanic).
    // GAP: "if you control a Nissa planeswalker, gain 2 life" — no named-planeswalker filter.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
