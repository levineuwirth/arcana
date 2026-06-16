//! Anikthea, Hand of Erebos — `{2}{W}{B}{G}` 4/4 Legendary Enchantment Creature — Demigod.
//! Menace; "Other enchantment creatures you control have menace."; "Whenever
//! Anikthea enters or attacks, exile up to one target non-Aura enchantment card
//! from your graveyard. Create a token that's a copy of that card, except it's a
//! 3/3 black Zombie creature in addition to its other types."
//!
//! "Other enchantment creatures have menace" is a pure static (GAP). The combined
//! enter/attack trigger is split into two TriggeredAbilityDefs. Each targets up to
//! one enchantment card in your graveyard (the "non-Aura" narrowing is GAP'd) and
//! exiles it. The "create a 3/3 black Zombie copy of that graveyard card" payoff is
//! not expressible — CopyPermanent copies a battlefield permanent and cannot apply
//! the 3/3-black-Zombie override — so that half is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

fn graveyard_enchantment_target() -> TargetRequirement {
    // GAP: "non-Aura" narrowing not expressible; targets any enchantment card.
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::new()
                .with_types(TypeLine::ENCHANTMENT.into())
                .controlled_by(ControllerConstraint::You),
        },
        count: TargetCount::UpTo(1),
        controller: None,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anikthea, Hand of Erebos");
    let demigod = reg.interner_mut().intern("Demigod");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demigod);

    // GAP: static — "other enchantment creatures you control have menace".

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_graveyard_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![graveyard_enchantment_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_graveyard_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![graveyard_enchantment_target()],
            }),
    )
}

fn exile_graveyard_enchantment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "create a 3/3 black Zombie token copy of that card" not expressible.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
