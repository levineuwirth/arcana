//! Battle Cry Goblin — `{1}{R}` 2/2 Goblin.
//! "{1}{R}: Goblins you control get +1/+0 and gain haste until end of turn."
//! Pack tactics (GAP keyword) — "Whenever this creature attacks, if you
//! attacked with creatures with total power 6 or greater this combat, create
//! a 1/1 red Goblin creature token that's tapped and attacking." (The
//! intervening-if and the "tapped and attacking" rider are GAP'd; a plain
//! 1/1 red Goblin token is created.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battle Cry Goblin");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Goblins you control get +1/+0 and gain haste until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_goblins,
            })
            // GAP: intervening-if "if you attacked with creatures with total
            // power 6 or greater this combat" has no condition helper.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_goblin_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_goblins(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Goblin")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::Pump {
            target: id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        });
    }
    effects
}

fn make_goblin_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token should enter "tapped and attacking" — no such primitive in
    // this catalog; a plain 1/1 red Goblin token is created instead.
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: goblin,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
