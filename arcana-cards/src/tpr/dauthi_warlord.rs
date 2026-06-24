//! Dauthi Warlord — `{1}{B}` */1 Dauthi Soldier with Shadow.
//!
//! Oracle:
//! * Shadow.
//! * Dauthi Warlord's power is equal to the number of creatures on the
//!   battlefield with shadow. (Installed at Layer 7a via an ETB self-CDA —
//!   `self_pt_cda` returning `(count of shadow creatures, 1)`. Asymmetric
//!   `*`/1; the `*` axis counts a KEYWORD (not a subtype name), which the
//!   no-registry compute resolves via a base-characteristic keyword filter.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauthi Warlord");
    let dauthi = reg.interner_mut().intern("Dauthi");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dauthi);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            shadow_creatures,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn shadow_creatures(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter = ObjectFilter::creature().with_keyword(KeywordAbility::Shadow);
    let n = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| filter.matches_base(o, s, who))
        .count() as i32;
    (n, 1)
}
