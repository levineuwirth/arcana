//! ED-E, Lonesome Eyebot — `{3}` 2/1 Legendary Artifact Creature — Robot,
//! Flying.
//!
//! Oracle:
//! * Flying.
//! * ED-E My Love — Whenever you attack, if the number of attacking
//!   creatures is greater than the number of quest counters on ED-E, put a
//!   quest counter on it.
//! * {2}, Sacrifice ED-E: Draw a card, then draw an additional card for
//!   each quest counter on ED-E.
//!
//! The Flying keyword and the sacrifice-draw activated ability are wired.
//! The activated ability draws 1 plus one per quest counter currently on
//! ED-E (read at resolution, before the sacrifice removes it from play —
//! the count is captured from the live source object).
//!
//! GAP (trigger): "Whenever you attack, if the number of attacking
//! creatures is greater than the number of quest counters on ED-E, put a
//! quest counter on it" — there is no "you attack" (attack-step
//! declaration) trigger condition in the documented surface, and the
//! intervening-if comparing attacker count to a counter count is not an
//! available predicate. The whole trigger is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("ED-E, Lonesome Eyebot");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, Sacrifice ED-E: Draw a card, then draw an additional \
                   card for each quest counter on ED-E."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: sac_draw,
        }),
    )
}

/// Draw 1 + one card per quest counter on the source.
fn sac_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let quest = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::Quest));
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1 + quest,
    }]
}
