//! Kuja, Genome Sorcerer // Trance Kuja, Fate Defied
//!
//! Front (Kuja, Genome Sorcerer, {2}{B}{R}, Legendary 3/4 Human Mutant Wizard):
//!   At the beginning of your end step, create a tapped 0/1 black Wizard creature token with
//!   "Whenever you cast a noncreature spell, this token deals 1 damage to each opponent."
//!   Then if you control four or more Wizards, transform Kuja.
//!   (GAP: token's triggered sub-ability "whenever you cast a noncreature spell, deal 1 damage"
//!    is not expressible on a TokenDefinition — token is minted without that ability.)
//!   (GAP: token "enters tapped" not a TokenDefinition field — token enters untapped.)
//!   "if you control four or more Wizards" gates the Transform at resolution time via
//!   conditions::you_control_subtype_at_least (resolution-time clause inside the end-step
//!   trigger, not a whole-trigger intervening-if — the token is created unconditionally).
//!
//! Back (Trance Kuja, Fate Defied, Legendary Avatar Wizard):
//!   Flare Star — If a Wizard you control would deal damage to a permanent or player, it deals
//!   double that damage instead. (GAP: replacement effect not modeled.)

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kuja, Genome Sorcerer");
    let human = reg.interner_mut().intern("Human");
    let mutant = reg.interner_mut().intern("Mutant");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mutant);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Trance Kuja, Fate Defied");
    let avatar = reg.interner_mut().intern("Avatar");
    let wizard2 = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar);
    back_subtypes.0.insert(wizard2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Wizard for token lookup at resolve time
    let _wizard_tok = reg.interner_mut().intern("Wizard");

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // At the beginning of your end step: create 0/1 black Wizard token, then transform
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: back-face-only replacement effect (Flare Star — Wizards deal double damage) not modeled.
    )
}

fn end_step_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(wizard_sym) = reg.interner().lookup("Wizard") else { return Vec::new(); };
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(wizard_sym);

    // Create a 0/1 black Wizard creature token.
    // GAP: token's sub-ability and "enters tapped" are not expressible in TokenDefinition.
    let token = TokenDefinition {
        name: wizard_sym,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    // "Then if you control four or more Wizards, transform Kuja." Resolution-time
    // gate via conditions::you_control_subtype_at_least (the token is created
    // unconditionally; only the Transform is conditional).
    let mut effects = vec![Effect::CreateToken { controller: trig.controller, token }];
    if conditions::you_control_subtype_at_least(state, reg, trig.controller, "Wizard", 4) {
        effects.push(Effect::Transform { target: trig.source });
    }
    effects
}
