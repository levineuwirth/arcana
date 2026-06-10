//! Spitting Slug — `{1}{G}{G}` 2/4 green Slug.
//! "Whenever this creature blocks or becomes blocked, you may pay
//! {1}{G}. If you do, this creature gains first strike until end of
//! turn. Otherwise, each creature blocking or blocked by this creature
//! gains first strike until end of turn."
//! The "otherwise" branch enumerates the paired combatants via
//! `script::blockers_of` + `script::attackers_blocked_by` and grants
//! first strike to each through OptionalPayment's `else_effect`.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spitting Slug");
    let slug = reg.interner_mut().intern("Slug");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(slug);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: first_strike_choice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn first_strike_choice(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Otherwise, each creature blocking or blocked by this creature gains
    // first strike" — both pairing directions, enumerated at resolution.
    let mut others = script::blockers_of(state, trig.source);
    others.extend(script::attackers_blocked_by(state, trig.source));
    let else_grants: Vec<Effect> = others
        .into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        })
        .collect();
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{G}").expect("valid cost")),
        then: Box::new(Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        }),
        else_effect: Some(Box::new(Effect::Sequence(else_grants))),
    }]
}
