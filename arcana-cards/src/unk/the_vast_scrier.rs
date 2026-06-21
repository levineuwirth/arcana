//! The Vast Scrier — `{1}{R}{W}{B}` 2/2 Legendary Human Cleric.
//!
//! Oracle:
//! * Flying
//! * Whenever The Vast Scrier attacks a player, you may put a Soldier,
//!   Warrior, or Wizard creature card from your hand onto the battlefield
//!   tapped and attacking that player. … If you don't put a card onto the
//!   battlefield this way, scry 2.
//!
//! The attack trigger fires on `SelfAttacks`; the put-from-hand is modeled
//! with `PutFromHandOntoBattlefieldTappedAttacking` over a Soldier/Warrior/
//! Wizard creature filter.
//!
//! GAP: the conditional fallback "If you don't put a card onto the
//! battlefield this way, scry 2" depends on whether the optional may-put
//! resolved — there is no primitive that branches on the result of a
//! PutFromHand pick, so the scry-2 fallback is omitted. The "tapped and
//! attacking that player" routing is engine-chosen.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Vast Scrier");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    // Pre-intern the put-filter subtypes so the resolver can look them up.
    let _soldier = reg.interner_mut().intern("Soldier");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: put_from_hand_attacking,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn put_from_hand_attacking(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Soldier") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Warrior") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Wizard") {
        syms.push(s);
    }
    let filter = ObjectFilter::creature().with_subtypes_any(syms);
    vec![Effect::PutFromHandOntoBattlefieldTappedAttacking {
        player: trig.controller,
        filter,
    }]
    // GAP: "If you don't put a card onto the battlefield this way, scry 2"
    // — conditional on the optional put resolving; not expressible.
}
