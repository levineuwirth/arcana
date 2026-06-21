//! Thousand-Faced Shadow — `{U}` 1/1 Human Ninja with Flying (and Ninjutsu).
//!
//! Oracle:
//! * Ninjutsu {2}{U}{U}. (Ninjutsu is not an expressible keyword — omitted.)
//! * Flying.
//! * When this creature enters from your hand, if it's attacking, create a
//!   token that's a copy of another target attacking creature. The token
//!   enters tapped and attacking.
//!   Modeled as an ETB trigger targeting another attacking creature and
//!   minting a copy via Effect::CopyPermanent.
//!   GAP: the "from your hand, if it's attacking" intervening-if is not
//!   expressible (no enters-from-hand / self-is-attacking condition), and the
//!   copy token cannot be made to enter tapped and attacking.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thousand-Faced Shadow");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "enters from your hand, if it's attacking" —
            //      no enters-from-hand / self-is-attacking condition.
            intervening_if: None,
            effect: copy_attacking_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn copy_attacking_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: the copy token should enter tapped and attacking — CopyPermanent
    //      mints the token but cannot stamp tapped-and-attacking.
    vec![Effect::CopyPermanent { target: *id }]
}
