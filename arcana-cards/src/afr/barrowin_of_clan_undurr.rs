//! Barrowin of Clan Undurr — `{2}{W}{B}` 3/3 Legendary Dwarf Cleric.
//! "When Barrowin enters, venture into the dungeon."
//! "Whenever Barrowin attacks, return up to one creature card with mana
//! value 3 or less from your graveyard to the battlefield if you've
//! completed a dungeon."
//!
//! "Venture into the dungeon" is not in the usable keyword surface (no
//! keyword emitted), but the ETB body maps to `Effect::Venture`. The
//! attack trigger returns up to one mv-3-or-less creature card from your
//! graveyard to the battlefield. The "if you've completed a dungeon"
//! conditional has no completed-dungeon predicate, so that gate is GAP'd
//! and the return fires whenever Barrowin attacks (the "up to one" still
//! lets the controller decline).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barrowin of Clan Undurr");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When Barrowin enters, venture into the dungeon."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_venture,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever Barrowin attacks, return up to one creature card
            // with mana value 3 or less from your graveyard to the
            // battlefield if you've completed a dungeon."
            // GAP: "if you've completed a dungeon" — no completed-dungeon
            // predicate; the return fires unconditionally on attack.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature().with_max_cmc(3),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_venture(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Venture { player: trig.controller }]
}

fn attack_reanimate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
