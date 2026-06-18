//! Kharis & The Beholder — `{1}{G}{G}{W}{W}` 1/20 Legendary Creature — Dragon
//! Eye Wizard with Flying.
//!
//! Oracle:
//! * Flying.
//! * When Kharis & The Beholder enters and at the beginning of your upkeep,
//!   create a 1/1 white Human creature token and make a charisma check.
//!   (Roll a d20.)
//!     - If the result plus the number of creatures you control is greater
//!       than 11, put a +1/+1 counter on each creature you control.
//!     - If the result is a natural 20, for each nonlegendary creature you
//!       control, create a token that's a copy of that creature.
//!
//! We model the always-on half of both triggers — minting the 1/1 white Human
//! token. The d20 "charisma check" (rolling a die and branching on the result)
//! has no engine primitive (no random d20 roll / dice mechanic in the effect
//! catalog), so the two roll-dependent bullet outcomes are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kharis & The Beholder");
    let dragon = reg.interner_mut().intern("Dragon");
    let eye = reg.interner_mut().intern("Eye");
    let wizard = reg.interner_mut().intern("Wizard");
    // Token subtype interned now so the resolver can recover it by name.
    let _human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(eye);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(20)),
        keywords: vec![arcana_core::effects::KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_token_and_charisma_check,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_token_and_charisma_check,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Create a 1/1 white Human creature token. The "make a charisma check"
/// d20 roll and its two result-dependent outcomes are not expressible.
fn make_token_and_charisma_check(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    // GAP: "make a charisma check (roll a d20)" — no d20 / dice-roll primitive,
    // so neither the ">11: +1/+1 counter on each creature" bullet nor the
    // "natural 20: copy each nonlegendary creature you control" bullet can be
    // gated on the roll result. We emit only the unconditional token mint.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: human,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
