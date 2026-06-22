//! The Rhystic Storyteller — `{2}{U}` 2/3 Legendary Human Wizard.
//! "Whenever you cast a spell with ten or more words of flavor text,
//! draw a card unless an opponent pays {1}."
//!
//! GAP: the Commander deck-construction static ("If The Rhystic
//! Storyteller is your Commander, your deck can't use sleeves.") is a
//! silver-border / format-construction rule with no engine effect.
//! GAP: the "ten or more words of flavor text" restriction on the
//! cast trigger is not expressible (no flavor-text predicate on an
//! ObjectFilter); the trigger fires on every spell you cast. The
//! draw-unless-an-opponent-pays-{1} body is wired via OptionalPayment.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Rhystic Storyteller");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: rhystic_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn rhystic_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "draw a card unless an opponent pays {1}" — prompt an opponent to
    // pay {1}; if they decline, you draw.
    let opp = match script::opponents(state, trig.controller).first() {
        Some(p) => *p,
        None => return vec![Effect::DrawCards { player: trig.controller, count: 1 }],
    };
    vec![Effect::OptionalPayment {
        chooser: opp,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        })),
    }]
}
