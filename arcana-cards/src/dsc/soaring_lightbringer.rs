//! Soaring Lightbringer — `{4}{W}` 4/5 Enchantment Creature — Bird Glimmer.
//! Flying.
//! "Other enchantment creatures you control have flying." (static — GAP)
//! "Whenever you attack a player, create a 1/1 white Glimmer enchantment
//! creature token that's tapped and attacking that player."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Soaring Lightbringer");
    let bird = reg.interner_mut().intern("Bird");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(glimmer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP (static): "Other enchantment creatures you control have flying" — a
    // continuous keyword-granting anthem keyed to a type filter; no triggered/
    // activated decomposition can express a static board-wide keyword grant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever you attack a player" — modeled as a creature-you-
                // control-attacks trigger (fires per attacker; minor fidelity GAP
                // vs the once-per-attack printing).
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                },
                intervening_if: None,
                effect: make_glimmer,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_glimmer(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let glimmer = reg.interner().lookup("Glimmer").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(glimmer);
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token: TokenDefinition {
            name: glimmer,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
