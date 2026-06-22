//! Anax and Cymede & Kynaios and Tiro — `{1}{R}{G}{W}{U}` 3/8 Legendary
//! Human Soldier with First strike and Vigilance.
//! "Heroic — Whenever you cast a spell that targets [this creature], draw a
//! card. Each player may put a land card from their hand onto the
//! battlefield, then each opponent who didn't draws a card." — wired as a
//! SelfBecomesTarget (caster: You) heroic trigger: you draw, then each player
//! may put a land from hand onto the battlefield. The "each opponent who
//! didn't draws a card" rider depends on per-player optional choices the
//! engine can't condition on after the fact, so it is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anax and Cymede & Kynaios and Tiro");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTarget {
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: heroic_draw_and_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn heroic_draw_and_land(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }];
    // "Each player may put a land card from their hand onto the battlefield."
    for p in script::all_players(state) {
        effects.push(Effect::PutFromHandOntoBattlefield {
            player: p,
            filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            tapped: false,
        });
    }
    // GAP: "then each opponent who didn't draws a card" — the engine can't
    // condition the draw on whether a player declined the optional land put.
    effects
}
