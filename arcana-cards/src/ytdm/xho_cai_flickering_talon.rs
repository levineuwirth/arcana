//! Xho Cai, Flickering Talon — `{U}{R}{W}` 2/4 Legendary Bird Monk with
//! Flying, Vigilance, Haste.
//! "When Xho Cai enters, the next noncreature spell you cast costs {1} less
//! to cast."
//! "Flurry — Whenever you cast your second spell each turn, exile up to one
//! target creature you control, then return it to the battlefield under its
//! owner's control."
//!
//! The three keywords are base characteristics. "Flurry" is an ability word.
//! The ETB cost-reduction ("next noncreature spell costs {1} less") is GAP'd
//! — there is no cost-reduction `NextCastRider` (only GainsHaste / counter /
//! copy riders exist). The second-spell trigger fires on `SpellCast`, gated in
//! the effect body to the 2nd cast (the count includes the just-cast spell,
//! the Captain Ripley Vance pattern), then blinks up to one target creature
//! you control via the established `ExilePermanent` + delayed
//! `ReturnFromExileToBattlefield` slow-blink (a documented partial: the return
//! lands at the next end step rather than immediately).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Xho Cai, Flickering Talon");
    let bird = reg.interner_mut().intern("Bird");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };
    // GAP: ETB "the next noncreature spell you cast costs {1} less to cast"
    // — no cost-reduction NextCastRider; the only riders are GainsHaste /
    // EntersWithPlusOneCounter / Copy / CopyTwice. The ETB trigger is omitted
    // rather than wired with a wrong rider.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: flurry_blink,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn flurry_blink(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Only fire on the 2nd spell each turn; the count includes the spell
    // whose cast triggered this ability.
    let count = script::spells_cast_this_turn(state, &ObjectFilter::new(), trig.controller);
    if count != 2 {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
