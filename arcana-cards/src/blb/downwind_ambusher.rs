//! Downwind Ambusher — `{3}{B}` 4/2 Skunk Assassin.
//!
//! "Flash
//!  When this creature enters, choose one —
//!  • Target creature an opponent controls gets -1/-1 until end of turn.
//!  • Destroy target creature an opponent controls that was dealt damage
//!    this turn."
//!
//! Decomposition: Flash keyword + an ETB trigger. The "choose one" modal
//! payload is NOT expressible on a triggered ability (modal dispatch is
//! spell-ability only), so we implement the first mode (-1/-1 to a
//! creature an opponent controls) and GAP the modal selection + the
//! second mode (destroy a damaged creature — the "was dealt damage this
//! turn" target restriction has no ObjectFilter form either).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Downwind Ambusher");
    let skunk = reg.interner_mut().intern("Skunk");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skunk);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "choose one" modal is not expressible on a triggered
            // ability. Mode 1 (-1/-1) is implemented; mode 2 (destroy a
            // creature dealt damage this turn) is GAP'd — both the choice
            // and the "was dealt damage this turn" target filter lack
            // engine support.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: minus_one_minus_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn minus_one_minus_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
