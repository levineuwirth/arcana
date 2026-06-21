//! Kimahri, Valiant Guardian — `{2}{G}{U}` 3/3 Legendary Cat Warrior.
//! Vigilance.
//! Ronso Rage — At the beginning of combat on your turn, put a +1/+1 counter
//! on Kimahri and tap target creature an opponent controls. Then you may have
//! Kimahri become a copy of that creature, except its name is Kimahri, it has
//! vigilance and this ability.
//!
//! The keyword is a base characteristic. The combat trigger puts a +1/+1
//! counter on Kimahri and taps a target opponent's creature; the
//! "become a copy of that creature" rider is not expressible (no make-an-
//! existing-object-become-a-copy primitive — CopyPermanent only MINTS a token),
//! so it is GAPped.

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kimahri, Valiant Guardian");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: ronso_rage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn ronso_rage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        out.push(Effect::Tap { target: *id });
    }
    // GAP: "Then you may have Kimahri become a copy of that creature, except …"
    // — no make-an-existing-object-become-a-copy effect (CopyPermanent mints a
    // token, it does not transform Kimahri itself).
    out
}
