//! Silent-Blade Oni — `{3}{U}{U}{B}{B}` 6/5 Demon Ninja.
//! Ninjutsu {4}{U}{B}.
//! "Whenever this creature deals combat damage to a player, look at that
//! player's hand. You may cast a spell from among those cards without paying
//! its mana cost."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silent-Blade Oni");
    let demon = reg.interner_mut().intern("Demon");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(ninja);
    // GAP (keyword): Ninjutsu {4}{U}{B} — not in the usable KeywordAbility set
    // and not expressible as an activated ability (return-an-unblocked-attacker
    // cost + put-from-hand-tapped-and-attacking from the Hand zone).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}{B}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: steal_a_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn steal_a_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at that player's hand. You may cast a spell from among those
    // cards without paying its mana cost." CastFromHandFree needs a specific
    // target id, but there is no way to post a may-choose pick over another
    // player's hand and feed the chosen card into a free cast.
    Vec::new()
}
