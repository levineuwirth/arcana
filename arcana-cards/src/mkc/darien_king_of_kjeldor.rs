//! Darien, King of Kjeldor — `{4}{W}{W}` 3/3 legendary white Human Soldier.
//! "Whenever you're dealt damage, you may create that many 1/1 white Soldier creature tokens."
//! GAP: "whenever you're dealt damage" trigger not directly in catalog; using
//! SelfIsDealtDamage as closest approximation (triggers on damage to the card itself,
//! not to the player). Using DamageDealt targeting you with source_filter=any as best-effort.
//! GAP: trig.damage_amount() gives count for token creation.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Darien, King of Kjeldor");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
                intervening_if: None,
                effect: on_dealt_damage_create_soldiers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dealt_damage_create_soldiers(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let soldier = reg.interner().lookup("Soldier")
        .expect("Soldier interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken { controller: trig.controller, token: token.clone() })
        .collect()
}
