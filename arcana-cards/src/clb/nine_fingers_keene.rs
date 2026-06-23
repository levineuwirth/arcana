//! Nine-Fingers Keene — `{1}{B}{G}{U}` 4/4 Legendary Creature —
//! Human Rogue.
//!
//! * Menace (keyword).
//! * "Ward—Pay 9 life." — a non-mana ward cost; `KeywordAbility::Ward`
//!   only carries a `ManaCost`, so this life-payment ward is not
//!   expressible. GAP'd (keyword omitted).
//! * "Whenever Nine-Fingers Keene deals combat damage to a player, look
//!   at the top nine cards of your library. You may put a Gate card from
//!   among them onto the battlefield. Then if you control nine or more
//!   Gates, put the rest into your hand. Otherwise, put the rest on the
//!   bottom of your library in a random order." — a combat-damage-to-a-
//!   player trigger. Wired via `Effect::DigTopN` (look at top 9, may take
//!   one Gate, rest to bottom in random order). Two fidelity gaps: the
//!   chosen Gate goes to HAND rather than the battlefield, and the
//!   conditional "if you control nine or more Gates, put the rest into
//!   your hand" branch is not expressible — the rest always goes to the
//!   bottom (`DigRest::BottomRandom`).

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nine-Fingers Keene");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    // Pre-intern the Gate subtype so the resolver's filter can look it up.
    let _gate = reg.interner_mut().intern("Gate");

    // Source-restrict the combat-damage trigger to THIS creature. There is no
    // self-id source filter, so for this uniquely-named legendary we filter by
    // name (a faithful proxy for "Nine-Fingers Keene deals combat damage").
    let self_name = reg.interner().lookup("Nine-Fingers Keene");
    let source_filter = ObjectFilter {
        name: self_name,
        ..ObjectFilter::new().controlled_by(ControllerConstraint::You)
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: keyword — "Ward—Pay 9 life" (non-mana ward not expressible).

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter,
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: dig_for_gate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dig_for_gate(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Gate");
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 9,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
