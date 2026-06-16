//! Isareth the Awakener — `{1}{B}{B}` Legendary 3/3 Human Wizard with
//! Deathtouch.
//!
//! Oracle text:
//! * Deathtouch.
//! * "Whenever Isareth attacks, you may pay {X}. When you do, return
//!   target creature card with mana value X from your graveyard to the
//!   battlefield with a corpse counter on it. If that creature would
//!   leave the battlefield, exile it instead of putting it anywhere
//!   else." — GAP: the whole reflexive payload is unexpressible.
//!   OptionalPaymentKind has no X-cost form; the reanimation target's
//!   mana value can't be tied to the paid X; and the
//!   leaves-the-battlefield replacement linkage to a corpse counter is
//!   not modeled. The SelfAttacks trigger is still wired (empty effect).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Isareth the Awakener");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_reanimate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}. When you do, return target creature card with
    // mana value X from your graveyard to the battlefield with a corpse
    // counter; if it would leave, exile it instead." OptionalPaymentKind
    // has no X-cost variant, the reanimation target's mv can't be tied to
    // X, and the leaves-the-battlefield replacement linkage is not modeled.
    Vec::new()
}
