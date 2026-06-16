//! Wrexial, the Risen Deep — `{3}{U}{U}{B}` 5/8 Legendary Kraken.
//!
//! * Islandwalk, swampwalk.
//! * Whenever Wrexial deals combat damage to a player, you may cast target
//!   instant or sorcery card from that player's graveyard without paying its
//!   mana cost. If that spell would be put into a graveyard, exile it instead.
//!   (// GAP: the "that player's graveyard" restriction is not enforced — the
//!   target is any instant/sorcery card in a graveyard; the "may" optionality
//!   and the "exile instead of graveyard" rider are also not modeled. Best-
//!   effort cast-from-graveyard.)

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrexial, the Risen Deep");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let island = reg.interner_mut().intern("Island");
    let swamp = reg.interner_mut().intern("Swamp");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Landwalk(island),
            KeywordAbility::Landwalk(swamp),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: cast_from_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn cast_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player's graveyard" restriction, the "may" optionality, and
    // the "exile instead of graveyard" rider are not modeled — best-effort
    // free cast-from-graveyard of the chosen instant/sorcery.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::CastFromGraveyard {
        player: trig.controller,
        target: *id,
    }]
}
