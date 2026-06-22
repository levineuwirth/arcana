//! Satya, Aetherflux Genius — `{1}{U}{R}{W}` 3/5 Legendary Human Artificer.
//! Menace, haste.
//! Whenever Satya attacks, create a tapped and attacking token that's a copy
//! of up to one other target nontoken creature you control. You get {E}{E}.
//! At the beginning of the next end step, sacrifice that token unless you pay
//! an amount of {E} equal to its mana value.
//!
//! The attack trigger is implemented as: copy the chosen creature (mints a
//! token via Effect::CopyPermanent) plus gain two energy. The "tapped and
//! attacking" rider and the "sacrifice unless you pay {E} = its mana value"
//! delayed clause are GAP'd — CreateTokenTappedAttacking takes a fixed
//! TokenDefinition (not a copy), and there is no energy-payment cost field.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Satya, Aetherflux Genius");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn on_attack(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        // GAP: the copy should enter tapped and attacking, and the
        // "sacrifice unless you pay {E} equal to its mana value" delayed
        // clause is unexpressible (no energy-payment cost). Mint the copy.
        effects.push(Effect::CopyPermanent { target: *id });
    }
    effects.push(Effect::GainEnergy {
        player: trig.controller,
        amount: 2,
    });
    effects
}
