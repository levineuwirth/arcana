//! Zur, Eternal Schemer — `{W}{U}{B}` 1/4 Legendary Creature —
//! Human Wizard.
//!
//! * Flying, lifelink.
//! * Enchantment creatures you control have deathtouch, lifelink, and
//!   hexproof. — GAP: a continuous static anthem granting keywords to a
//!   filtered set you control (no trigger word, no cost). Not expressible
//!   as a triggered/activated ability.
//! * `{1}{W}: Target non-Aura enchantment you control becomes a creature
//!   in addition to its other types and has base power and base toughness
//!   each equal to its mana value.`
//!   Wired (partial): the chosen enchantment becomes a creature (additive
//!   type) permanently.
//!   GAP: the "base power and base toughness equal to its mana value"
//!   rider — no mana-value script helper to compute the dynamic base P/T
//!   per object. The "non-Aura" restriction is a fidelity GAP (no Aura
//!   subtype exclusion in the demonstrated filter surface).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zur, Eternal Schemer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W}: Target non-Aura enchantment you control becomes a creature in \
                   addition to its other types and has base power and base toughness each \
                   equal to its mana value."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types(TypeLine::ENCHANTMENT.into())
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: animate_enchantment,
        }),
    )
}

fn animate_enchantment(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddType {
        target: *id,
        types: TypeLine::CREATURE.into(),
        duration: Duration::Permanent,
    }]
}
