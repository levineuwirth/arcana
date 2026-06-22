//! Regna, the Redeemer — `{5}{W}` 4/4 Legendary Angel with Flying.
//!
//! Oracle:
//! * Partner with Krav, the Unredeemed (When this creature enters,
//!   target player may put Krav into their hand from their library, then
//!   shuffle.) — "Partner with" / "Partner" are not usable KeywordAbility
//!   variants; only the ETB tutor-helper is modeled (target player may
//!   tutor the named partner to hand; the "may" is the resolution-time
//!   choice on `TutorToHand`).
//! * Flying — keyword line.
//! * At the beginning of each end step, if your team gained life this
//!   turn, create two 1/1 white Warrior creature tokens. — modeled as a
//!   StepBegins(End, Any) trigger; the intervening-if uses
//!   `you_gained_life_this_turn` as the closest single-player
//!   approximation of "your team gained life" (multiplayer team
//!   accounting is GAP'd).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::effects::TokenDefinition;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Regna, the Redeemer");
    let angel = reg.interner_mut().intern("Angel");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: "Partner with" / "Partner" not expressible KeywordAbility variants.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_partner_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_gained_life),
                effect: make_warriors,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_partner_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let nm = reg.interner().lookup("Krav, the Unredeemed");
    vec![Effect::TutorToHand {
        player: *p,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: false,
    }]
}

fn if_gained_life(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // GAP fidelity: "your team gained life this turn" — team accounting is
    // not modeled; approximated as "you gained life this turn".
    conditions::you_gained_life_this_turn(s, you)
}

fn make_warriors(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}
