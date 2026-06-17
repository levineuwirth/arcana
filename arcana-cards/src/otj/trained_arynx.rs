//! Trained Arynx — `{1}{W}` 3/1 Cat Beast Mount.
//!
//! Whenever this creature attacks while saddled, it gains first strike until
//! end of turn. Scry 1.
//! Saddle 2.
//!
//! Saddle and Scry are not expressible keyword variants (GAP keyword line).
//! The attacks trigger is wired (SelfAttacks) granting first strike + Scry 1,
//! but the "while saddled" gate is not expressible so it over-fires (GAP).
//! The Saddle 2 activated ability (tap creatures with total power >= 2 to
//! become saddled) has no expressible cost/effect and is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trained Arynx");
    let cat = reg.interner_mut().intern("Cat");
    let beast = reg.interner_mut().intern("Beast");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(beast);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Saddle and Scry (as a keyword) are not expressible variants.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "while saddled" gate not expressible — over-fires on every
            // attack.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_first_strike_scry,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_first_strike_scry(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
        Effect::Scry {
            player: trig.controller,
            count: 1,
        },
    ]
}
