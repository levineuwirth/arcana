//! Wyll of the Celestial Pact — `{3}{R}{R}{W}` 5/5 Legendary red/white Human Warlock.
//! When this creature specializes, you may sacrifice another creature or an artifact.
//! When you do, return target creature card from your graveyard to the battlefield.
//! It gains haste. If its mana value is 4 or greater, sacrifice it at the beginning
//! of your next end step.
//!
//! GAP: `TriggerCondition::Specializes` does not exist in the catalog. Using
//! `SelfEntersBattlefield` as closest available trigger; the specialization trigger
//! will not fire correctly.
//!
//! The "you may sacrifice another creature or an artifact. When you do, return target
//! creature card …" gate is wired as an optional sacrifice payment (sacrifice a creature
//! or artifact, then reanimate the target with haste). Minor over-inclusion: the selection
//! can't exclude the source ("another"), so Wyll itself is technically offerable.
//!
//! GAP: Conditional "if its mana value is 4 or greater, sacrifice at beginning of
//! next end step" — mana value check on a just-returned permanent is not expressible
//! via `script::*`; the delayed sacrifice is omitted.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wyll of the Celestial Pact");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: TriggerCondition::Specializes not in catalog; using
                // SelfEntersBattlefield as placeholder.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: specializes_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn specializes_reanimate(
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
    // "you may sacrifice another creature or an artifact. When you do, return
    // target creature card from your graveyard to the battlefield. It gains
    // haste."
    // GAP: Conditional delayed sacrifice if mana value >= 4 not expressible via script::*.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::CreatureOrArtifact),
        then: Box::new(Effect::Sequence(vec![
            Effect::ReturnFromGraveyardToBattlefield { target: *id },
            Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Haste,
                duration: Duration::WhileSourceOnBattlefield,
            },
        ])),
        else_effect: None,
    }]
}
