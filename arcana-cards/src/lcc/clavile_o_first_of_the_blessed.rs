//! Clavileño, First of the Blessed — `{1}{W}{B}` 2/2 Legendary
//! Vampire Cleric. "Whenever you attack, target attacking Vampire
//! that isn't a Demon becomes a Demon in addition to its other types.
//! It gains 'When this creature dies, draw a card and create a tapped
//! 4/3 white and black Vampire Demon creature token with flying.'"
//!
//! GAP: trigger — "Whenever you attack" (the whole-combat attack
//! trigger) is approximated with `CreatureAttacks` filtered to your
//! creatures (fires per attacker rather than once per combat). The
//! target restriction "Vampire that isn't a Demon" is narrowed to the
//! Vampire subtype; the "isn't a Demon" exclusion is not expressible
//! via the documented `ObjectFilter` API. The granted dies-ability's
//! token is interned at register time.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    GRANTED_TRIGGER_ID_BASE, PendingTrigger, TriggerCondition, TriggerFrequency,
    TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clavileño, First of the Blessed");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    // Interned now so the granted dies-trigger resolver can mint the
    // Vampire Demon token via the read-only interner at resolve time.
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    let vampire_filter = arcana_core::script::subtype_filter(reg, "Vampire");
    let attack_filter = arcana_core::targets::ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: attack_filter,
                },
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(vampire_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let granted = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::SelfDies,
        intervening_if: None,
        effect: granted_on_dies,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    // GAP: "becomes a Demon" grants the Demon creature SUBTYPE; the
    // documented `Effect::AddType` only adds TypeLine types (CREATURE /
    // ARTIFACT / …), not subtypes, so the Demon-subtype grant is not
    // expressible. We still grant the dies-triggered ability.
    vec![
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(granted),
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}

fn granted_on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").unwrap_or_default();
    let demon = reg.interner().lookup("Demon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(demon);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![arcana_core::effects::KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: the token should enter tapped; CreateToken offers no
    // enters-tapped flag in the documented API.
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
